'use client'
import MergeDetail from "@/components/MergeDetail";
import { useEffect, useState } from "react";


export default function MRDetailPage( { params }: { params: { id: string } }) {
    const [mrDetail, setMrDetail] = useState([]);
    useEffect(() => {
        const fetchData = async () => {
            try {
                const res = await fetch(`/api/mr/${params.id}/detail`);
                const response = await res.json();
                const data = response.data.data;
                setMrDetail(data);
            } catch (error) {
                console.error('Error fetching data:', error);
            }
        };
        fetchData();
    }, [params.id]);

    return (
        <div>
            <MergeDetail mrDetail={mrDetail}/>
        </div>
    )
}
